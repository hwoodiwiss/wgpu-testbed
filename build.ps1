cargo build

Push-Location ".\wgpu-testbed-lib"
$env:RUSTFLAGS = "--cfg=web_sys_unstable_apis"
wasm-pack build
Pop-Location

Push-Location ".\wgpu-testbed-webapp"
Remove-Item "./node_modules" -Recurse -ErrorAction SilentlyContinue
yarn install
yarn build
Pop-Location