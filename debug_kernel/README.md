# Debug kernel
This kernel example use the write debug to write a "hello-world", this kernel use the Alcotest approach for testing.

- Build `debug_kernel.wasm` run:

```shell
# build and generate to debug_kernel.wasm
cargo make wasm-debug-kernel

# copy the generated of debug_kernel.wasm to the root directory,
# rename it to write-debug.wasm and use wasm-strip to strip its size
cp target/wasm32-unknown-unknown/release/debug_kernel.wasm debug_kernel.wasm

wasm-strip debug_kernel.wasm
```

- Alcotest

This kernel use the Alcotest in `tezos/src/lib_scoru_wasm/test` approach. The current working branch is: https://gitlab.com/marigold/tezos/-/tree/quyen@wasm_debug_tool/src/lib_scoru_wasm/test

The folder: `wasm_marigold_kernels` contains the `debug_kernel.wasm` after using `wasm-strip`. The file `test_marigold_wasm_pvm.ml` contains the test for it.
