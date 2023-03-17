# Example 0: Debug Kernel

In our first kernel, we will demonstrate how to write debug messages
and read from the shared inbox.

## Running the example

First, compile the kernel to WASM with Cargo:
<!-- $MDX skip -->
```sh
$ cargo build --release --target wasm32-unknown-unknown
```

Then you can execute the kernel locally against the provided inputs and commands:
```sh
$ octez-smart-rollup-wasm-debugger \
> ../target/wasm32-unknown-unknown/release/debug_kernel.wasm \
> --inputs ./inputs.json \
> --commands ./commands.json
Loaded 2 inputs at level 0
Hello from kernel!
Message from the runtime: [0, 1]
Evaluation took 136121 ticks so far
Status: Evaluating
Internal_status: Evaluation succeeded
Hello from kernel!
Evaluation took 11000000000 ticks so far
Status: Evaluating
Internal_status: Eval
Message from the runtime: [0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
Evaluation took 125166 ticks so far
Status: Evaluating
Internal_status: Evaluation succeeded
Hello from kernel!
Message from the user: hello from bob.
Evaluation took 10999875322 ticks so far
Status: Evaluating
Internal_status: Evaluation succeeded
Hello from kernel!
Message from the user: hello from alice.
Evaluation took 11000000000 ticks so far
Status: Evaluating
Internal_status: Eval
```

Additionally, you can omit the `--commands` flag to enter a REPL mode and
explore the execution of the kernel interactively. Try it out!
