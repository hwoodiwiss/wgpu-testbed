#! /usr/bin/env pwsh
#Requires -Version 7.0
#Requires -PSEdition Core

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true
$originalRustFlags = $env:RUSTFLAGS
$originalEncodedRustFlags = $env:CARGO_ENCODED_RUSTFLAGS
try {
$env:RUSTFLAGS = $null
$env:CARGO_ENCODED_RUSTFLAGS = $null
cargo clippy --workspace --all-targets -- -D warnings

Push-Location ".\wgpu-testbed-lib"
try {
wasm-pack build --release
} finally { Pop-Location }

Push-Location ".\wgpu-testbed-webapp"
try {
Remove-Item "./node_modules" -Recurse -ErrorAction SilentlyContinue
Remove-Item "./dist" -Recurse -ErrorAction SilentlyContinue
npm i
npm run build
} finally { Pop-Location }
} finally {
    $env:RUSTFLAGS = $originalRustFlags
    $env:CARGO_ENCODED_RUSTFLAGS = $originalEncodedRustFlags
}
