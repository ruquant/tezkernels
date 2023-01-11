# How to interact with MondayNet?

- Why MondayNet?

  The MondayNet test is one of the periodic Tezos testnets that support SORU. More information can be found in [https://teztnets.xyz/mondaynet-about](https://teztnets.xyz/mondaynet-about).
- What are explorers support MondayNet?
  - [https://status.teztnets.xyz/](https://status.teztnets.xyz/)
  - [https://explorus.functori.com/scoru](https://explorus.functori.com/scoru)

- What are wallets support MondayNet?
  - Temple wallet

- How to **setup** MondayNet?
  
  - From Tezos source code: Please follow the tutorial on https://teztnets.xyz/mondaynet-about. Note that, the parameters of branch checkout and the network is changing because the branch restarts each Monday.
    
    For instance: Download and compile the source code as following:
    ```shell
        cd tezos
        # the branch c49508dc will change in the future
        git checkout c49508dc 
        eval $(opam env)
        make build-dev-deps
        make
    ```

- How to **config** `octez-node` on MondayNet?
  
    ```shell
      # config octez-node with the mondaynet network
      # the network: https://teztnets.xyz/mondaynet-2023-01-09 will change in the future
      ./octez-node config init https://teztnets.xyz/mondaynet-2023-01-09
    ```
- How to **run** `octez-node`?  
  - From Tezos source code
    ```shell
    ./octez-node run --rpc-addr 127.0.0.1:8732
    ```
   Note that, we need to check if there is already `octez-node` running in the processor by checking:
    ```shell
    ps -aux | grep tezos
    ```
   if there is, we need to kill it and then we can run the `octez-node` as follow:
    ```shell
    kill -9 <id>
    ```

- How to **add account** on MondayNet?

  Note that, we only show how to use the commands from Tezos source code, other approach will be similar.
  ```shell
  # generate new account
  ./octez-client gen keys alice
  # show 
  ./octez-client list known addresses
  ```

- How to add **faucet** to the account?
  
   The faucet for MondayNet can be found at https://teztnets.xyz/mondaynet-about.

   For instance, `alice` has an account: `tz1NCwYf8HF1V2nft3w657uwNQZPyMHA1xEM`. To add the faucet for `alice`, copy her address to the box in the section `Or fund any address` and request `6001tz` for her.

   To check if `alice` receives the fund:

   ```shell
   ./octez-client get balance for alice
   ```

   For the account from Temple wallet, connect to the wallet and then request the fund for it.
