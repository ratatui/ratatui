[CmdletBinding()]
param(
    [string]$WasiSdk,
    [string]$WitBindgen,
    [string]$WitFile,
    [string]$World,
    [string]$OutDir,
    [string]$OutName
)

$ErrorActionPreference = "Stop"

if (-not $WasiSdk) {
    $WasiSdk = if ($env:WASI_SDK) { $env:WASI_SDK } else { "C:\wasi-sdk" }
}
if (-not $WitBindgen) {
    $WitBindgen = if ($env:WIT_BINDGEN) { $env:WIT_BINDGEN } else { "wit-bindgen" }
}
if (-not $WitFile) {
    $WitFile = if ($env:WIT_FILE) { $env:WIT_FILE } else { "..\..\..\ratatui-wasm\wit\widget.wit" }
}
if (-not $World) {
    $World = if ($env:WORLD) { $env:WORLD } else { "wasm-widget" }
}
if (-not $OutDir) {
    $OutDir = if ($env:OUT_DIR) { $env:OUT_DIR } else { "generated" }
}
if (-not $OutName) {
    $OutName = if ($env:OUT_NAME) { $env:OUT_NAME } else { "hello_c" }
}

$Clang = Join-Path $WasiSdk "bin\wasm32-wasip2-clang.exe"

New-Item -ItemType Directory -Force -Path $OutDir | Out-Null

& $WitBindgen c $WitFile --world $World --out-dir $OutDir
& $Clang -c "$OutDir\wasm_widget.c" -o wasm_widget.o -I $OutDir
& $Clang -c main.c -o main.o -I $OutDir
& $Clang main.o wasm_widget.o "$OutDir\wasm_widget_component_type.o" -o "$OutName.wasm" "-Wl,--no-entry"

Write-Output "Built $OutName.wasm"
