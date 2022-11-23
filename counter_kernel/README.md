# Counter kerenel

This is a counter kernel example, this kernel purposely test with the tool `octez-wasm-repl`.

- Build
```shell
cargo make wasm-counter-kernel
# copy generated wasm to counter wasm folder
cp target/wasm32-unknown-unknown/release/counter_kernel.wasm counter_kernel/wasm/counter_kernel.wasm
# call wasm-strip
~/wabt/bin/wasm-strip counter_kernel/wasm/counter_kernel.wasm
```

## Use `octez-wasm-repl-alpha`

The detail of this tool can be found at this [tutorial](https://tezos.gitlab.io/alpha/smart_rollups.html). 

Syntax:
```
./octez-wasm-repl-alpha <.wasm> --inputs <.json>
```

where:
- .wasm: is the wasm file
- .json: inbox messages

If no input is given it assumes to be empty.

### Test `counter_kernel.wasm`

- input: this is a sequence of inboxes, where an inbox is a set of messages. Below is a valid input file that define one inbox, contain 1 message `payload: 0`:

`input.json`
```json
[
  [
    {
      "payload" : "0"
    }    
  ]
]
```
The sender, source, destination (is the default address of the rollup) is empty, it will take the default addresses.


```shell
cd tezos
git checkout quyen@wasm_debug_tool
eval $(opam env)
make build-unreleased
# copy the test_counter.wasm from the kernels repo 
./octez-wasm-repl-alpha src/lib_scoru_wasm/test/marigold_wasm_kernels/counter_kernel.wasm --inputs input.json
# initial state, waiting for input
> show status
Status: Waiting for input
Internal_status: Collect
# Loading inputs
> load inputs
Loaded 1 inputs at level 0
# state change to start to process
> show status
Status: Evaluating
Internal_status: Start
# show the input input in inbox
>  show inbox
Inbox has 3 messages:
{ raw_level: 0;
  counter: 0
  payload: Start_of_level }
{ raw_level: 0;
  counter: 1
  payload: 
{ "internal_inbox_message_kind": "transfer", "payload": { "int": "0" },
  "sender": "KT18amZmM5W7qDWVt2pH6uj7sCEd3kbzLrHT",
  "source": "tz1Ke2h7sDdakHJQh8WX4Z372du1KChsksyU",
  "destination": "scr1AFyXAWFS3c6S2D617o8NGRvazoMJPEw6s" } }
{ raw_level: 0;
  counter: 2
  payload: End_of_level }
# evaluation state
> step kernel_run
Evaluation took 11000000000 ticks so far
Status: Evaluating
Internal_status: Start
# evaluation the inbox, inbox now change to collect state
> step inbox
Evaluation took 33000000000 ticks so far
Status: Waiting for input
Internal_status: Collect
# show the status of kernel
> show status
Status: Waiting for input
Internal_status: Collect
```

TODO: show outbox

### Test `echo.wasm`
The example provided by the tutorial, the kernel source code of `echo.wasm` can be found at: https://gitlab.com/tezos/kernel/-/tree/echo-kernel/,
the `input_echo.json` is the one in the tutorial, the `echo.wasm` kernel take input as an `External` (ref [code](https://gitlab.com/tezos/kernel/-/blob/echo-kernel/echo_kernel/src/lib.rs#L59)), so it will read the first array in the `input_echo.json`:

```shell
# a part of input_echo.json
[
    {
      "external":
      "0000000023030b01d1a37c088a1221b636bb5fccb35e05181038ba7c000000000764656661756c74"
    }
  ],
  ....
```

```shell
./octez-wasm-repl-alpha src/proto_alpha/lib_protocol/test/integration/wasm_kernel/echo.wasm --inputs input_echo.json
> show status
Status: Waiting for input
Internal_status: Collect
> load inputs
Loaded 1 inputs at level 0
> show status
Status: Evaluating
Internal_status: Start
> show inbox
Inbox has 3 messages:
{ raw_level: 0;
  counter: 0
  payload: Start_of_level }
{ raw_level: 0;
  counter: 1
  payload: 0000000023030b01d1a37c088a1221b636bb5fccb35e05181038ba7c000000000764656661756c74 }
{ raw_level: 0;
  counter: 2
  payload: End_of_level }
> step kernel_run
Evaluation took 11000000000 ticks so far
Status: Evaluating
Internal_status: Start
> show key /store/key
Key not found
> step result
Evaluation took 2367 ticks so far
Status: Evaluating
Internal_status: Eval
> step kernel_run
Evaluation took 10999997633 ticks so far
Status: Evaluating
Internal_status: Start
> step inbox
Evaluation took 11000000000 ticks so far
Status: Waiting for input
Internal_status: Collect
> show outbox
Unknown command `show outbox`
```
### Feedback: 
- the current output is different than the one show in the document
- there is no `show outbox`, `show memory at p for 1 bytes`
- when looking for key `show key /store/key` it is not found.

The `octez-wasm-repl-alpha` (the one generated from tezos/master). 
- There is no `--rollup` option 
- There is no `--help`, or `man`, etc.
- It is better if the guide provide and explain the `.wasm` and the `.json` input that it used to show the result in the guide. Because there is no way to check or understand from the output.
