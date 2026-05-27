@echo off
rem cargo install wasm-pack

pushd "%~dp0"

wasm-pack build --target web --out-dir pkg/wasm
python -m http.server --directory pkg 8000

popd
