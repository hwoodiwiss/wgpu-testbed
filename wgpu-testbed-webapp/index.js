import * as wasm from "../wgpu-testbed-lib/pkg/wgpu_testbed_lib";

wasm.run_wasm();
window.sceneViewer = wasm;

function configure(changes) {
  const status = JSON.parse(wasm.navigation_status());
  if (status.settings) wasm.set_navigation_settings(JSON.stringify({ ...status.settings, ...changes }));
}
const speed = document.getElementById("nav-speed");
const mode = document.getElementById("nav-mode");
speed.addEventListener("change", () => {
  const value = Number(speed.value);
  if (Number.isFinite(value) && value > 0) configure({ speed: value, automatic_speed: false });
});
mode.addEventListener("change", () => configure({ mode: mode.value }));
document.getElementById("nav-frame").addEventListener("click", () => wasm.navigation_command("Frame"));
document.getElementById("nav-reset").addEventListener("click", () => wasm.navigation_command("Reset"));
setInterval(() => {
  const state = JSON.parse(wasm.navigation_status());
  if (!state.settings) return;
  document.getElementById("nav-status").textContent = `${state.settings.mode} · ${state.settings.speed.toPrecision(4)} units/s · ${state.device} · ${state.active ? "Active" : "Focus canvas + Enter, click, or press controller A"}${state.captured ? " · Mouse captured (Escape releases)" : ""}`;
  document.getElementById("nav-help").textContent = state.help;
  if (document.activeElement !== speed) speed.value = state.settings.speed;
  if (document.activeElement !== mode) mode.value = state.settings.mode;
}, 250);
