# Marigold kernel

## Prerequired

- `wasm-strip` is a part of the [WebAssembly Binary Toolkit](https://github.com/WebAssembly/wabt) to strip down the size of wasm kernel.

### Install Rust
The rust version is `1.60.0`

```shell
# [install rust]
wget https://sh.rustup.rs/rustup-init.sh
chmod +x rustup-init.sh
./rustup-init.sh --profile minimal --default-toolchain 1.60.0 -y
# [source cargo]
. $HOME/.cargo/env
```

- Add `wasm32-unknown-unknown` target:

```shell
rustup target add wasm32-unknown-unknown
```
## Kernels example available

### Counter kernel
This example showing how to storing (read/write) an `Int`.

- Build
```shell
cargo make wasm-counter-kernel
```
This will export the wasm at the directory `[target/wasm32-unknown-unknown/release/counter_kernel.wasm]`

- Strip the size of the export kernel
```
~wabt/bin/wasm-strip target/wasm32-unknown-unknown/release/counter_kernel.wasm
```
- Rust unit test
```shell
cargo test
```
### Debug kernel
Debug kernel use the function `WasmHost::write_debug`, currently this function write the given number of bytes to debug log and return nothing. The purpose of this kernel showing that we need a return value to check the message that we write.

- Build
```shell
cargo make wasm-debug-kernel
```

### Hello world kernel
This example showing how to storing (read/write) a `String`.

- Build
```shell
cargo make hello-world-kernel
```

TODO: xf fill-in

### Output kernel
This example showing how to use input/output message.

- Build
```shell
cargo make wasm-output-kernel
```

- Rust unit test
```shell
cargo test
```

## Gitbook

Gitbook of [pistachio](https://app.gitbook.com/o/Gayxsw4YmVrLK4YRDlmi/s/bQv7Nn2dfUKSfyTtFM1M/pistachio/kernel-in-scoru).

# Footnotes

The logo of this project is the [Pistachio icons created by Freepik - Flaticon](https://www.flaticon.com/free-icons/pistachio).
