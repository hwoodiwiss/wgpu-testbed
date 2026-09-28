import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./tests",
  timeout: 60_000,
  expect: { timeout: 15_000 },
  // Each viewer owns a continuous GPU loop. Serialize tests, isolate contexts.
  workers: 1,
  forbidOnly: !!process.env.CI,
  retries: 0,
  reporter: [["list"], ["html", { open: "never" }]],
  use: {
    baseURL: "http://127.0.0.1:5199",
    browserName: "chromium",
    // Use full Chromium, including its compositor, rather than headless-shell.
    channel: "chromium",
    viewport: { width: 1200, height: 1000 },
    trace: "retain-on-failure",
    screenshot: "only-on-failure",
    launchOptions: {
      executablePath: process.env.BROWSER_EXECUTABLE || undefined,
      args: ["--enable-unsafe-webgpu", "--enable-unsafe-swiftshader"],
    },
  },
  projects: [
    { name: "web", metadata: { host: "/web/" } },
    { name: "bundler", metadata: { host: "/bundler/" } },
  ],
  webServer: {
    command: "node tests/support/server.mjs",
    url: "http://127.0.0.1:5199/health",
    reuseExistingServer: false,
    timeout: 30_000,
  },
});
