import { test as base, expect } from "@playwright/test";
import { PNG } from "pngjs";

export { expect };

export class Viewer {
  constructor(page) {
    this.page = page;
    this.canvas = page.locator("canvas");
  }

  status() {
    return this.page.evaluate(() =>
      JSON.parse(window.sceneViewer.navigation_status()),
    );
  }

  async ready() {
    await expect
      .poll(
        () =>
          this.page.evaluate(() => {
            if (!window.sceneViewer) return false;
            return !!JSON.parse(window.sceneViewer.navigation_status())
              .settings;
          }),
        { message: "WASM renderer/navigation initialization" },
      )
      .toBe(true);
    await expect(this.canvas).toBeVisible();
    await expect
      .poll(() => this.canvas.evaluate((c) => c.width > 1 && c.height > 1), {
        message: "Canvas backing store has a real layout size",
      })
      .toBe(true);
  }

  async activate() {
    await this.canvas.focus();
    await this.page.keyboard.press("Enter");
    await expect.poll(async () => (await this.status()).active).toBe(true);
  }

  async release() {
    await this.page.keyboard.press("Escape");
    await expect.poll(async () => (await this.status()).active).toBe(false);
  }

  command(action) {
    return this.page.evaluate(
      (action) => window.sceneViewer.navigation_command(action),
      action,
    );
  }

  async settings(changes) {
    await this.page.evaluate((changes) => {
      const { settings } = JSON.parse(window.sceneViewer.navigation_status());
      window.sceneViewer.set_navigation_settings(
        JSON.stringify({ ...settings, ...changes }),
      );
    }, changes);
    await expect
      .poll(async () => (await this.status()).settings)
      .toMatchObject(changes);
  }

  async move(key = "w", distance = 0.2) {
    const before = (await this.status()).eye;
    await this.page.keyboard.down(key);
    try {
      await expect
        .poll(async () => displacement((await this.status()).eye, before), {
          message: `Camera translates while ${key} is held`,
        })
        .toBeGreaterThan(distance);
    } finally {
      await this.page.keyboard.up(key);
    }
  }

  screenshot() {
    // Capture compositor pixels, not the cleared WebGPU drawing buffer.
    return this.canvas.screenshot({ animations: "disabled" });
  }
}

export function displacement(a, b) {
  return Math.hypot(...a.map((value, i) => value - b[i]));
}

export function foregroundFraction(buffer) {
  const { data, width, height } = PNG.sync.read(buffer);
  // Ignore focus outlines and antialiasing-dependent color buckets. A flat-shaded
  // object can legitimately have only one color distinct from the background.
  const background = [...data.slice((width + 1) * 4, (width + 1) * 4 + 3)];
  let foreground = 0;
  let sampled = 0;
  for (let y = 4; y < height - 4; y++) {
    for (let x = 4; x < width - 4; x++) {
      const i = (y * width + x) * 4;
      if (
        background.some(
          (value, channel) => Math.abs(data[i + channel] - value) > 30,
        )
      )
        foreground++;
      sampled++;
    }
  }
  return foreground / sampled;
}

export function changedPixelFraction(before, after) {
  const a = PNG.sync.read(before);
  const b = PNG.sync.read(after);
  expect([b.width, b.height]).toEqual([a.width, a.height]);
  let changed = 0;
  for (let i = 0; i < a.data.length; i += 4) {
    if (
      [0, 1, 2].some(
        (channel) => Math.abs(a.data[i + channel] - b.data[i + channel]) > 30,
      )
    )
      changed++;
  }
  return changed / (a.width * a.height);
}

export const test = base.extend({
  diagnostics: [
    async ({ page }, use, testInfo) => {
      const logs = [];
      const errors = [];
      page.on("console", (message) =>
        logs.push(`${message.type()}: ${message.text()}`),
      );
      page.on("pageerror", (error) => {
        errors.push(error.message);
        logs.push(error.stack);
      });
      page.on("requestfailed", (request) =>
        logs.push(
          `request failed: ${request.url()} ${request.failure()?.errorText}`,
        ),
      );
      await use();
      const state = await page
        .evaluate(() => ({
          url: location.href,
          focus: document.activeElement?.outerHTML,
          canvas: [...document.querySelectorAll("canvas")].map((c) => ({
            width: c.width,
            height: c.height,
          })),
          navigation: window.sceneViewer?.navigation_status(),
        }))
        .catch((error) => ({ error: error.message }));
      await testInfo.attach("browser.log", {
        body: logs.join("\n"),
        contentType: "text/plain",
      });
      await testInfo.attach("viewer-state.json", {
        body: JSON.stringify(state, null, 2),
        contentType: "application/json",
      });
      expect(errors, "Uncaught browser errors").toEqual([]);
    },
    { auto: true },
  ],
  viewer: async ({ page }, use, testInfo) => {
    await page.goto(testInfo.project.metadata.host);
    const supported = await page.evaluate(
      async () => !!(navigator.gpu && (await navigator.gpu.requestAdapter())),
    );
    expect(
      supported,
      "WebGPU adapter required; see browser.log and trace",
    ).toBe(true);
    const viewer = new Viewer(page);
    await viewer.ready();
    await use(viewer);
  },
});
