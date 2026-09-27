# WGPU Testbed

## Scene navigation

Native and WASM support keyboard-only, mouse+keyboard, and Xbox-style controllers.
Click/focus the canvas and press Enter, or press controller A. WASD/left stick moves,
arrows/mouse/right stick looks, E/Q or RT/LT changes height, O/View switches Fly/Orbit,
and F/Y frames the scene. Escape/B releases input.

See [controls, settings and verification](crates/scene-navigation/CONTROLS.md).
The repository owns its local `scene-navigation` workspace crate and its
deterministic and browser tests. It builds and evolves independently.

## This is a testbed for playing with WGPU and related technology across native and WebAssembly targets

Originally based on sotrh's Learn WGPU tutorial, available [here](https://sotrh.github.io/learn-wgpu/).

## Links

### Latest: https://agreeable-dune-08facd403.azurestaticapps.net/

### Stable: https://victorious-grass-0945f1903.azurestaticapps.net/

The testbed webapp is automatically built and deployed on PRs (Latest) and pushes to main (Stable and Latest).

Other resources:

- [WebGPU Spec](https://www.w3.org/TR/webgpu/)
- [WGSL Spec](https://www.w3.org/TR/WGSL/)
- [gfx-rs/wgpu](https://github.com/gfx-rs/wgpu)
