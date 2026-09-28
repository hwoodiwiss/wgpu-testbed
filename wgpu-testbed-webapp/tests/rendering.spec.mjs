import {
  test,
  expect,
  foregroundFraction,
  changedPixelFraction,
  displacement,
} from "./support/viewer.mjs";

test("scene geometry renders and the displayed image changes after turning", async ({
  viewer,
  page,
}, testInfo) => {
  const initial = (await viewer.status()).eye;
  await viewer.command("Frame");
  await expect
    .poll(async () => (await viewer.status()).eye)
    .not.toEqual(initial);
  let before;
  await expect
    .poll(
      async () => {
        before = await viewer.screenshot();
        return foregroundFraction(before);
      },
      { message: "Scene geometry covers at least 1% of the composited canvas" },
    )
    .toBeGreaterThan(0.01);
  await testInfo.attach("scene-before.png", {
    body: before,
    contentType: "image/png",
  });
  await viewer.activate();
  const directionBefore = (await viewer.status()).target;
  const box = await viewer.canvas.boundingBox();
  await page.mouse.move(box.x + 100, box.y + 100);
  await page.mouse.down({ button: "right" });
  await page.mouse.move(box.x + 280, box.y + 100, { steps: 10 });
  await page.mouse.up({ button: "right" });
  await expect
    .poll(async () =>
      displacement((await viewer.status()).target, directionBefore),
    )
    .toBeGreaterThan(0.05);
  await viewer.release();
  let after;
  await expect
    .poll(
      async () => {
        after = await viewer.screenshot();
        return changedPixelFraction(before, after);
      },
      { message: "Turning changes at least 1% of displayed pixels" },
    )
    .toBeGreaterThan(0.01);
  await testInfo.attach("scene-after.png", {
    body: after,
    contentType: "image/png",
  });
});
