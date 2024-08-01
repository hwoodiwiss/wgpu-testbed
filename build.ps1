#! /usr/bin/env pwsh
#Requires -Version 7.0
#Requires -PSEdition Core

cargo clippy -- -D warnings
cargo build --release

Push-Location ".\wgpu-testbed-lib"
$env:RUSTFLAGS = "--cfg=web_sys_unstable_apis"
wasm-pack build --release
Pop-Location

Push-Location ".\wgpu-testbed-webapp"
Remove-Item "./node_modules" -Recurse -ErrorAction SilentlyContinue
Remove-Item "./dist" -Recurse -ErrorAction SilentlyContinue
yarn install
yarn build
Pop-Location