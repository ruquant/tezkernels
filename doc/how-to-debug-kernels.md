# How to debug kernel?

- What **tool** can I use to debug kernel?
  
  The tool `octez-wasm-alpha-repl` is used to test kernels during its development, without replying on starting a rollup on a test network.

- What does `octez-wasm-alpha-repl` debug?
  
  There is a maximum number of ticks allow in an PVM, `octez-wasm-repl` bound to this max (11,000,000,000 ticks). After reading the kernel file, it will parse, type check and link for safety before feeding the kernel to the PVM, then installation into a tree for the PVM interpreter.

- How to **get** it?
  
  From the Tezos source code compile it with the option
  
  ```shell
  cd tezos
  eval $(opam env)
  make build-unreleased
  ```
- How to **use** it?

  ```shell
  ./octez-wasm-repl-alpha kernel.wasm --inputs input.json
  ```
  where:
  - `kernel.wasm`: is the kernel that you want to debug. This kernel need to be smaller than **32kB**.
  - `input.json`: is the input that you want your kernel to evaluate.

- How to **strip** the size of kernel?
  
  You can use the `wasm-strip` is a part of the [WebAssembly Binary Toolkit](https://github.com/WebAssembly/wabt) to strip down the size of wasm kernel.
  ```shell
  ~wabt/bin/wasm-strip target/wasm32-unknown-unknown/release/kernel.wasm
  ```

- How to **write** input.json?
  
  For example, the `input.json`:
  ```json
  [
    [
        {
          "external":
          "0000000023030b01d1a37c088a1221b636bb5fccb35e05181038ba7c000000000764656661756c74"
        }
    ],
    [
        {
          "payload": "0",
          "sender": "KT1ThEdxfUcWUwqsdergy3QnbCWGHSUHeHJq",
          "source": "tz1RjtZUVeLhADFHDL8UwDZA6vjWWhojpu5w",
          "destination: "scr1HLXM32GacPNDrhHDLAssZG88eWqCUbyLF",
        },
        {
          "payload": "Pair Unit False"
        }
    ]
  ]
  ```
  
  Where the input is either: external or internal.

  - External:

    It is a hex representation of the payload.
  - Internal:
    - Internal transfers:
      - payload: Michelson data (`Pair Unit False` or `0` in the example above)
      - sender: the contract hash of the originated contract for the rollup
      - source: the implicit acccount sending the message
      - destination: the smart rollup address

      If the `sender, source, destination` are not given, it will be taken from the default addresses.

- How to **use commands** in `octez-wasm-alpha-repl`?
  
  ```shell
  ./octez-wasm-repl-alpha kernel.wasm --inputs input.json
  # showing the status of the repl
  > show status
  # load the first inbox from the file given with the option --inputs
  # putting Start_of_level and End_of_level  before (resp. after) for these inputs.
  > load inputs
  # show the input that loaded
  > show inbox
  # start a kernel run evaluation
  > step kernel_run
  # inspect the memory by stopping the PVM
  # before its snapshot internal state
  > step result
  # evaluate the whole inbox
  > step inbox
  ```
