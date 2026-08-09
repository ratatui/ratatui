#!/bin/sh
set -e

WASI_SDK="${WASI_SDK:-/opt/wasi-sdk}"
WIT_BINDGEN="${WIT_BINDGEN:-wit-bindgen}"
WIT_FILE="${WIT_FILE:-../../../ratatui-wasm/wit/widget.wit}"
WORLD="${WORLD:-wasm-widget}"
OUT_DIR="${OUT_DIR:-generated}"
OUT_NAME="${OUT_NAME:-hello_c}"

CLANG="$WASI_SDK/bin/wasm32-wasip2-clang"

mkdir -p "$OUT_DIR"

$WIT_BINDGEN c "$WIT_FILE" --world "$WORLD" --out-dir "$OUT_DIR"
$CLANG -c "$OUT_DIR/wasm_widget.c" -o wasm_widget.o -I "$OUT_DIR"
$CLANG -c main.c -o main.o -I "$OUT_DIR"
$CLANG main.o wasm_widget.o "$OUT_DIR/wasm_widget_component_type.o" -o "$OUT_NAME.wasm" -Wl,--no-entry

echo "Built $OUT_NAME.wasm"
