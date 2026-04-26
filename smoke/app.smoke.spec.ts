import { expect, test } from "@playwright/test";

test("opens a DAO save, resets inventory drafts, commits, and saves a copy", async ({ page }) => {
  await page.goto("/");

  await expect(page.getByRole("heading", { name: "No save open" })).toBeVisible();
  await page.getByRole("button", { name: /open save/i }).click();

  await expect(page.getByRole("button", { name: "Characters" })).toBeVisible();
  await expect(page.getByText("Aedan").first()).toBeVisible();

  await page.getByRole("button", { name: "Inventory" }).click();
  await expect(page.getByRole("button", { name: /Starfang/ })).toBeVisible();
  await page.getByRole("button", { name: /Starfang/ }).click();

  const stackSize = page.getByLabel("Stack Size");
  await expect(stackSize).toHaveValue("3");
  await stackSize.fill("12");
  await page.getByLabel("Money").fill("999");

  await page.getByRole("button", { name: /reset drafts/i }).click();
  await expect(stackSize).toHaveValue("3");
  await expect(page.getByLabel("Money")).toHaveValue("100");

  await stackSize.fill("12");
  await page.getByLabel("Money").fill("999");
  await page.getByRole("button", { name: /commit changes/i }).click();

  await expect(page.getByText("Unsaved changes")).toBeVisible();
  await expect(page.getByRole("button", { name: /save as/i })).toBeEnabled();
  await page.getByRole("button", { name: /save as/i }).click();
  await expect(page.getByText("Saved copy ready")).toBeVisible();
});

test("opens a DA2 save and shows plot flags", async ({ page }) => {
  await page.addInitScript(() => {
    localStorage.setItem("smokeGame", "da2");
  });
  await page.goto("/");
  await page.getByRole("button", { name: /open save/i }).click();

  await expect(page.getByRole("button", { name: "Plot Flags" })).toBeVisible();
  await page.getByRole("button", { name: "Plot Flags" }).click();
  await expect(page.getByRole("heading", { name: "DA2 Plot Flags" })).toBeVisible();
  await expect(page.getByText("Act 1 major choice")).toBeVisible();
});
