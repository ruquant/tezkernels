# Installer Kernel

- Why and how do I use the **installer kernel**?
  
  When originating a WebAssembely Smart Rollup on Tezos, we must specify the full kernel (as hex-encoding WebAssembly) to be executed. Since this operation takes place on layer-1, this places a limit of **32KB** on the size of the origination kernel.
  
  Most useful kernel will be significantly larger than this, however, due to requiring certain runtime attributes such as: allocation, formatting, and cryptographic verification. The **installer kernels** were provided - that are small enough to be used as an origination kernel, that contain logic to install the kernel actually desired by the rollup operators. This is done through the **kernel upgrade mechanism**.

  You can imagine the installer kernels will slit the target kernel to a smaller size (a list of files hex, kernel binaries), and then with the kernel upgrade mechanism, when originate smart rollup, the first for instance we call it [`installer.hex`](https://gitlab.com/emturner/tx-client/-/blob/main/installer.hex) that is fit in an layer-1 operation, will be used as a bootstrap for this kernel, the rest of the files hex of the kernel will need to copy to the smart rollup node directory, to allows the reveal channel to find them when the installer requests.

  Example:
  ```sh
  # originate a smart rollup with the installer.hex
  # where the `pair string (ticket string)` is Michelson type of
  # contract mint_and_deposit_to_rollup.tz in tx-client project
  ./octez-client originate sc rollup from "${OPERATOR_ADDR}" \
  of kind wasm_2_0_0 \
  of type 'pair string (ticket string)' \
  with kernel $(cat installer.hex) \
  --burn-cap 999
  # initialise the smart rollup config
  ./octez-sc-rollup-node-alpha --base-dir "${OCLIENT_DIR}" \
  init operator config for "${SOR_ADDR}" \
  with operators "${OPERATOR_ADDR}" \
  --data-dir "${ROLLUP_NODE_DIR}"
  ```
  Assume that you have a folder name `wasm_2_0_0` contain a list of files hex of the kernel that you generated after using the installer kernel (for example, the folder [wasm_2_0_0](https://gitlab.com/emturner/tx-client/-/tree/main/wasm_2_0_0) in the tx-client). Then before running the smart rollup node, ensure that you copy the contents of `wasm_2_0_0` to `${ROLLUP_NODE_DIR/wasm_2_0_0`}. The copied files correspond to the contents of the kernel binary - and allows the reveal channel to find them when the installer requests:
  ```sh
  mkdir ${ROLLUP_NODE_DIR}/wasm_2_0_0
  cp ./wasm_2_0_0/* ${ROLLUP_NODE_DIR}/wasm_2_0_0
  ```
  You can now run the smart rollup node
  ```sh
  ./octez-sc-rollup-node-alpha -d "${OCLIENT_DIR}" run --data-dir "${ROLLUP_NODE_DIR}"
  ```

- Do we have an example of how to use installer kernel?
  
  A tutorial of how to use installer kernel with tx-kernel can be found at [this document](https://gitlab.com/emturner/tx-client). From the section `TX kernel client` onwards of tx-client please follow its tutorial. In this guide, we provide the pre-setup to run smoothly the tutorial:
  - Tezos side (test on [MondayNet](https://teztnets.xyz/mondaynet-about))
    - Run octez-node ([How to interact with MondayNet](doc/how-to-mondaynet.md))
      ```sh
      # config node
      ./octez-node config init --network https://teztnets.xyz/mondaynet-2023-01-02
      # run node
      ./octez-node run --rpc-addr 127.0.0.1:8732
      ```
    - Originate contract example [`mint_and_deposit_to_rollup.tz`](https://gitlab.com/emturner/tx-client/-/blob/main/mint_and_deposit_to_rollup.tz) (provided in tx-client project). You may need to wait for awhile for the node to bootstrap
      ```sh
      # Create account and fund tz
      ./octez-client gen keys alice
      # check
      ./octez-client list known addresses
      # Fund tz for tz1aM7PE7xgMyG1jY46KBtCy7Ykfk4rLa4fk (the alice just created)
      # open https://faucet.mondaynet-2023-01-02.teztnets.xyz/
      # copy the address into the section `Or fund any address` and request 6001 tz
      # you may be waiting for awhile to see the fund arrive
      # check the balance
      ./octez-client get balance for alice
      # Originate the example contract with alice account
      ./octez-client -d $HOME/.tezos-node originate \
      contract mint_and_deposit_ticket transferring 0 from \
      alice running $(cat ~/tx-client/mint_and_deposit_ticket.tz) \
      --burn-cap 0.1155 --init "Unit"
      # Save the contract address after originate successful
      ```
  - On tx-client
    ```sh
    # clone the project
    git clone https://gitlab.com/emturner/tx-client
    # build with required rustc 1.60
    cargo run --
    ```
    - Setting up the client
      ```sh
      # create alias for cli
      alias tx-client='cargo run --'
      # test cli tx-client
      tx-client --help
      # prepare tx_config='tx-client.json', show $export -p
      export TX_CONFIG='tx-client.json'
      ```
