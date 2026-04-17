import { test, expect } from "@playwright/test";

test.describe("FleetReserve fullstack UI e2e", () => {
  test("login and key navigation flow", async ({ page }) => {
    await page.goto("http://localhost:8081/login");

    await expect(page.getByText("FleetReserve Login")).toBeVisible();
    await page.getByLabel("Username").fill("admin");
    await page.getByLabel("Password").fill("FleetReserveHttpTest#2026");
    await page.getByRole("button", { name: "Sign In" }).click();

    // After successful login, dashboard and nav should be visible.
    await expect(page.getByText("Dashboard")).toBeVisible();
    await expect(page.getByRole("link", { name: "Vehicles" })).toBeVisible();
    await expect(page.getByRole("link", { name: "Calendar" })).toBeVisible();

    // Navigate to core pages to validate FE↔BE wiring through UI.
    await page.getByRole("link", { name: "Vehicles" }).click();
    await expect(page.getByText("Vehicle Management")).toBeVisible();

    await page.getByRole("link", { name: "Reservations" }).click();
    await expect(page.getByText("Reservations")).toBeVisible();

    await page.getByRole("link", { name: "Admin" }).click();
    await expect(page.getByText("Administration")).toBeVisible();
  });
});
