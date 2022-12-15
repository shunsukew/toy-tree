# toy-tree
Simple toy tree command
![Image](/images/screenshot.png)

## How to run with Wasm
### Prerequisites
Install wasm runtime (Wasmer or Wasmtime)
- Wasmer https://wasmer.io/
- Wasmtime https://docs.wasmtime.dev/cli-install.html

### Steps
1. Add wasm32-wasi toolchain
```
rustup target add wasm32-wasi
```

2. Build
```
cargo build --target wasm32-wasi
```

3. Execute
**Wasmer**
```
# This will show nothing because of sandbox environment
wasmer run ./target/wasm32-wasi/release/toy-tree.wasm -- --recursive

# Add capability
wasmer run --dir=. ./target/wasm32-wasi/release/toy-tree.wasm -- --recursive
```

**Wasmtime**
```
# This will show nothing because of sandbox environment
wasmtime ./target/wasm32-wasi/release/toy-tree.wasm -- --recursive

# Add capability
wasmtime --dir=. ./target/wasm32-wasi/release/toy-tree.wasm -- --recursive
```
