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
Message from the runtime: Internal(StartOfLevel)
Evaluation took 557667 ticks so far
Status: Evaluating
Internal_status: Evaluation succeeded
Hello from kernel!
Evaluation took 11000000000 ticks so far
Status: Evaluating
Internal_status: Eval
Message from the runtime: Internal(InfoPerLevel(InfoPerLevel { predecessor_timestamp: 1970-01-01T00:00:00Z, predecessor: BlockHash("BKiHLREqU3JkXfzEDYAkmmfX48gBDtYhMrpA98s7Aq4SzbUAB6M") }))
Evaluation took 589985 ticks so far
Status: Evaluating
Internal_status: Evaluation succeeded
Hello from kernel!
Message from the user: hello from bob.
Evaluation took 10999407354 ticks so far
Status: Evaluating
Internal_status: Evaluation succeeded
Hello from kernel!
Message from the user: hello from alice.
Evaluation took 11000000000 ticks so far
Status: Evaluating
Internal_status: Eval
```

Additionally, you can omit the `--commands` flag to enter a REPL mode and
explore the execution of the kernel interactively. Try it out!
