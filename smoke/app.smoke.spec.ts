import { expect, test } from "@playwright/test";
import type { Page } from "@playwright/test";

async function openMockSave(page: Page, game: "dao" | "da2" = "dao") {
  await page.addInitScript((selectedGame) => {
    localStorage.setItem("smokeGame", selectedGame);
  }, game);
  await page.goto("/");
  await page.getByRole("button", { name: /open save/i }).click();
}

test("opens a DAO save, resets inventory drafts, and applies drafts before saving a copy", async ({ page }) => {
  await openMockSave(page);

  await expect(page.getByRole("button", { name: "Characters" })).toBeVisible();
  await expect(page.getByText("Aedan").first()).toBeVisible();
  await expect(page.getByRole("button", { name: "Plot Flags" })).toHaveCount(0);
  await expect(page.getByRole("button", { name: /reset drafts/i })).toHaveCount(1);

  await page.getByRole("button", { name: "Inventory" }).click();
  await expect(page.getByRole("button", { name: /Starfang/ })).toBeVisible();
  await page.getByRole("button", { name: /Starfang/ }).click();

  const stackSize = page.getByLabel("Stack Size");
  await expect(stackSize).toHaveValue("3");
  await stackSize.fill("12");
  await page.getByLabel("Party gold").fill("999");

  await page.getByRole("button", { name: /reset drafts/i }).click();
  await expect(stackSize).toHaveValue("3");
  await expect(page.getByLabel("Party gold")).toHaveValue("100");

  await stackSize.fill("12");
  await page.getByLabel("Party gold").fill("999");

  await expect(page.getByRole("button", { name: /save as/i })).toBeEnabled();
  await page.getByRole("button", { name: /save as/i }).click();
  await expect(page.getByText("You have unsaved drafts. Apply them before saving?")).toBeVisible();
  await page.getByRole("button", { name: "Cancel" }).click();
  await expect(stackSize).toHaveValue("12");
  await expect(page.getByLabel("Party gold")).toHaveValue("999");

  await page.getByRole("button", { name: /save as/i }).click();
  await page.getByRole("button", { name: "Apply drafts and save" }).click();
  await expect(page.getByText("Saved copy ready")).toBeVisible();
});

test("edits character progress, attributes, and point pools with reset and commit", async ({ page }) => {
  await openMockSave(page);

  const level = page.getByLabel("Level");
  const strength = page.getByLabel("Strength");
  const talentPoints = page.getByLabel("Talent Points");

  await expect(level).toHaveValue("1");
  await expect(strength).toHaveValue("10");
  await expect(talentPoints).toHaveValue("3");

  await level.fill("8");
  await strength.fill("22");
  await talentPoints.fill("6");
  await page.getByRole("button", { name: /reset drafts/i }).click();
  await expect(level).toHaveValue("1");
  await expect(strength).toHaveValue("10");
  await expect(talentPoints).toHaveValue("3");

  await level.fill("8");
  await strength.fill("22");
  await talentPoints.fill("6");
  await page.getByRole("button", { name: /apply drafts/i }).click();
  await expect(page.getByText("Unsaved changes")).toBeVisible();
  await expect(level).toHaveValue("8");
  await expect(strength).toHaveValue("22");
  await expect(talentPoints).toHaveValue("6");

  await level.fill("9");
  await page.getByRole("button", { name: /reset drafts/i }).click();
  await expect(level).toHaveValue("8");
});

test("edits equipped item metadata through the character equipment tab", async ({ page }) => {
  await openMockSave(page);

  await page.getByRole("button", { name: "Equipment" }).click();
  await page.getByRole("button", { name: /Equipped Sword/ }).click();

  const itemLevel = page.getByLabel("Item Level");
  await expect(itemLevel).toHaveValue("0");
  await itemLevel.fill("5");
  await page.getByRole("button", { name: /apply drafts/i }).click();

  await expect(page.getByText("Unsaved changes")).toBeVisible();
  await expect(itemLevel).toHaveValue("5");

  await itemLevel.fill("6");
  await page.getByRole("button", { name: /reset drafts/i }).click();
  await expect(itemLevel).toHaveValue("5");
});

test("edits item metadata and properties in the backpack", async ({ page }) => {
  await openMockSave(page);

  await page.getByRole("button", { name: "Inventory" }).click();
  await page.getByRole("button", { name: /Starfang/ }).click();

  await page.getByLabel("Item Level").fill("4");
  await page.getByRole("button", { name: "+ add property" }).click();
  await page.locator(".prop-add-form select").selectOption("9");
  await page.locator(".prop-add-form").getByPlaceholder("Power").fill("3");
  await page.locator(".prop-add-form").getByRole("button", { name: "Add" }).click();

  await page.getByRole("button", { name: /apply drafts/i }).click();
  await expect(page.getByText("Unsaved changes")).toBeVisible();
  await expect(page.getByLabel("Item Level")).toHaveValue("4");
  await expect(page.locator(".prop-chips-label")).toContainText("(2)");
  await expect(page.getByText("Fire Damage")).toBeVisible();
});

test("resets and commits crafting recipe changes", async ({ page }) => {
  await openMockSave(page);

  await page.getByRole("button", { name: "Recipes" }).click();
  await page.getByRole("region", { name: "Potions" }).getByRole("button", { name: /Potions/ }).click();
  const lyriumPotion = page.getByLabel("Lyrium Potion");
  await expect(page.getByLabel("Health Poultice")).toBeChecked();
  await expect(lyriumPotion).not.toBeChecked();

  await lyriumPotion.check();
  await page.getByRole("button", { name: /reset drafts/i }).click();
  await expect(lyriumPotion).not.toBeChecked();

  await lyriumPotion.check();
  await page.getByRole("button", { name: /apply drafts/i }).click();
  await expect(page.getByText("Unsaved changes")).toBeVisible();
  await expect(lyriumPotion).toBeChecked();
});

test("opens a DA2 save and shows plot flags", async ({ page }) => {
  await openMockSave(page, "da2");

  await expect(page.getByRole("button", { name: "Plot Flags" })).toBeVisible();
  await page.getByRole("button", { name: "Plot Flags" }).click();
  await expect(page.getByRole("heading", { name: "Plot Flags" })).toBeVisible();
  await expect(page.getByLabel("Plot flag categories").getByRole("button", { name: "Warden", exact: true })).toBeVisible();
  await expect(page.getByRole("heading", { name: "Origin" })).toBeVisible();
  await expect(page.getByRole("radio", { name: "Human Noble" })).toBeVisible();
  await expect(page.getByLabel("Plot flag categories").getByRole("button", { name: "Broken Circle" })).toBeVisible();
});

test("resets, commits, and saves DA2 plot flag drafts", async ({ page }) => {
  await openMockSave(page, "da2");

  await page.getByRole("button", { name: "Plot Flags" }).click();
  const humanNoble = page.getByRole("radio", { name: "Human Noble" });
  const templars = page.getByRole("radio", { name: "Templars" });

  await humanNoble.click();
  await templars.click();
  await page.getByRole("button", { name: /reset drafts/i }).click();
  await expect(humanNoble).toHaveAttribute("aria-checked", "false");
  await expect(templars).toHaveAttribute("aria-checked", "false");

  await humanNoble.click();
  await templars.click();
  await page.getByRole("button", { name: /apply drafts/i }).click();
  await expect(page.getByText("Unsaved changes")).toBeVisible();
  await expect(humanNoble).toHaveAttribute("aria-checked", "true");
  await expect(templars).toHaveAttribute("aria-checked", "true");

  await page.getByRole("button", { name: /save as/i }).click();
  await expect(page.getByText("Saved copy ready")).toBeVisible();
});

test("shows recoverable errors for invalid opens and failed commits", async ({ page }) => {
  await page.addInitScript(() => {
    localStorage.setItem("smokeInvalidValidation", "1");
  });
  await page.goto("/");
  await page.getByRole("button", { name: /open save/i }).click();
  await expect(page.getByText(/validation reported an invalid save structure/i)).toBeVisible();
  await expect(page.getByRole("heading", { name: "No save open" })).toBeVisible();
  await page.getByRole("button", { name: "Dismiss" }).click();

  await page.evaluate(() => {
    localStorage.removeItem("smokeInvalidValidation");
    localStorage.setItem("smokeFailCommand", "set_money");
  });
  await page.getByRole("button", { name: /open save/i }).click();
  await page.getByRole("button", { name: "Inventory" }).click();
  await page.getByLabel("Party gold").fill("777");
  await page.getByRole("button", { name: /apply drafts/i }).click();
  await expect(page.getByText(/io: Mocked set_money failure/)).toBeVisible();
  await expect(page.getByText("Saved copy ready")).toBeVisible();
});
