# Marigold kernel

[[_TOC_]]

## Prerequired

- `wasm-strip` is a part of the [WebAssembly Binary Toolkit](https://github.com/WebAssembly/wabt) to strip down the size of wasm kernel.

### Install Rust

The official website to install rust can be found at: <https://www.rust-lang.org/tools/install>.

The rust version is `1.66.0`

```shell
# [install rust]
wget https://sh.rustup.rs/rustup-init.sh
chmod +x rustup-init.sh
./rustup-init.sh --profile minimal --default-toolchain 1.66.0 -y
# [source cargo]
. $HOME/.cargo/env
```

- Add `wasm32-unknown-unknown` target:

```shell
rustup target add wasm32-unknown-unknown
```

- To run unit tests `wasm-bindgen-test`, install the test runner:

```shell
cargo install wasm-bindgen-cli
```

- To run makefile, install [`cargo-make`](https://github.com/sagiegurari/cargo-make)
  
```shell
cargo install --force cargo-make
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

### Hello kernel
This example shows how to use [Capn' Proto](https://capnproto.org) for safe and efficient message decoding
on DAC/DAL input.
- Build
```
cargo make wasm-hello-kernel
```

## `octez-wasm-repl` debug tool for kernel

As REPL (read-eval-print-loop) is an interactive environment, the `octez-wasm-repl` is the tool to evaluate the WASM PVM without running any Tezos node in the background. It has been designed for interact and test the kernel in a local environment.

In the Pistachio-gitbook you can find the tutorial of:
- [How to debug wasm kernels](https://marigold-proto.gitbook.io/proto-gitbook/smart-optimistic-rollup/how-to-mondaynet)

## Interact kernel with SORU

Currently, the MondayNet test is one of the periodic Tezos testnets. More information can be found in <https://teztnets.xyz/mondaynet-about>

In the Pistachio-gitbook, you can find the tutorials of:
- [How to interact with Mondaynet](https://marigold-proto.gitbook.io/proto-gitbook/smart-optimistic-rollup/how-to-mondaynet)
- [How to interact with SORU](https://marigold-proto.gitbook.io/proto-gitbook/smart-optimistic-rollup/how-to-mondaynet) 

## Pistachio-gitbook

[Pistachio-gitbook](https://marigold-proto.gitbook.io/proto-gitbook/).

# Footnotes

The logo of this project is the [Pistachio icons created by Freepik - Flaticon](https://www.flaticon.com/free-icons/pistachio).
