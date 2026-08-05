$ErrorActionPreference = "Stop"

$WasiSdk = if ($env:WASI_SDK) { $env:WASI_SDK } else { "C:\wasi-sdk" }
$Clang = Join-Path $WasiSdk "bin\wasm32-wasip2-clang.exe"
$WitBindgen = "wit-bindgen"

New-Item -ItemType Directory -Force -Path generated | Out-Null

& $WitBindgen c ..\..\..\ratatui-wasm\wit\widget.wit --world wasm-widget --out-dir generated
& $Clang -c generated\wasm_widget.c -o wasm_widget.o -I generated
& $Clang -c main.c -o main.o -I generated
& $Clang main.o wasm_widget.o generated\wasm_widget_component_type.o -o hello_c.wasm "-Wl,--no-entry"

Write-Output "Built hello_c.wasm"
