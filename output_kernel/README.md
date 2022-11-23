# Output kernel
This kernel is use for rust unit testing read and write output message. 

- Build and run test, the generated `.wasm` can be found at: `target/wasm32-unknown-unknown/release/*.wasm`

```shell
# build and generate output-kernel.wasm
cargo make wasm-output-kernel

# run test
cargo test
```
