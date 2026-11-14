import { expect, test } from "@playwright/test";

const HARNESS = "/tests/harness/index.html";

test("reduce motion does not start a transition on every element", async ({ page }) => {
    await page.goto(HARNESS);
    await expect(page.getByRole("button", { name: "设置", exact: true })).toBeVisible();
    await page.getByRole("button", { name: "设置", exact: true }).click();
    // The checkbox itself is opacity 0; the label is the visible control.
    await page.locator("label.switch", { hasText: "减少动态效果" }).click();
    await expect(page.locator(".app")).toHaveClass(/reduce-motion/);
    await expect(page.locator(".nav-item").first()).toHaveCSS("transition-property", "none");

    await page.evaluate(() => {
        const probe = window as unknown as { __transitions: number };
        probe.__transitions = 0;
        document.addEventListener("transitionrun", () => {
            probe.__transitions += 1;
        }, true);
    });
    await page.locator(".player-bar").getByRole("button", { name: "播放队列" }).click();
    await expect(page.locator(".app")).toHaveClass(/queue-open/);
    const opened = await page.evaluate(() => {
        const probe = window as unknown as { __transitions: number };
        return { transitions: probe.__transitions, elements: document.querySelectorAll("*").length };
    });
    expect(opened.elements).toBeGreaterThan(50);
    expect(opened.transitions).toBeLessThan(12);
    await expect(page.locator(".queue")).toHaveCSS("transition-property", "none");
});
