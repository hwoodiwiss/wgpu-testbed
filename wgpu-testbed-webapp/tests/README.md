# Browser tests

Playwright Test runs the real WASM renderer with a fresh browser context per test.
Named tests are independent; one worker avoids competing continuous GPU loops.
All fixtures, assets and dependencies belong to this repository.

## Setup and run

From `wgpu-testbed-lib`:

```powershell
wasm-pack build --release --target web --out-dir pkg-web
wasm-pack build --release --target bundler --out-dir pkg
```

From `wgpu-testbed-webapp`:

```powershell
npm ci
npx playwright install chromium
npm run build
npm run test:navigation
```

The `web` project uses `fixtures/web.html`; `bundler` uses the built webapp.
Playwright manages the read-only asset server. Checkout names do not select test
behavior and no sibling repository is needed.

```powershell
npx playwright test --project=web gamepad.spec.mjs
npx playwright test --project=bundler --headed
npx playwright test --ui
npx playwright show-report
```

Use the Chromium installed by the pinned Playwright version locally and in CI.
`BROWSER_EXECUTABLE` is an optional explicit local browser override. Chromium's
software fallback is allowed; an unavailable WebGPU adapter fails explicitly.

## Structure and assertions

- `navigation.spec.mjs`: keyboard, mouse drag, focus/Tab, rebinding.
- `gamepad.spec.mjs`: standard mapping, neutral activation, disconnect, unsupported mapping.
- `rendering.spec.mjs`: framed cube scene and displayed-image changes.
- `support/viewer.mjs`: viewer operations, state assertions and automatic diagnostics.
- `support/gamepad.mjs`: Gamepad-shaped snapshots and polling acknowledgements.
- `support/server.mjs`: explicit asset routes for both packaging formats.

Tests wait for observable state instead of arbitrary delays or frame counts.
Rendering captures compositor screenshots, not a possibly cleared WebGPU drawing
buffer. The smoke check requires contrasting interior geometry and a meaningful
image change after the camera turns. It is not a cross-driver golden image test.
Physical Xbox and pointer-capture acceptance remains a separate hardware check.

Failures retain traces, screenshots, console/request logs and navigation state.
CI uploads `playwright-report/` and `test-results/`; generated reports are gitignored.
