# How to interact with MondayNet?

- Why MondayNet?

  The MondayNet test is one of the periodic Tezos testnets that support SORU. More information can be found in [https://teztnets.xyz/mondaynet-about](https://teztnets.xyz/mondaynet-about).
- What are explorers support MondayNet?
  - [https://status.teztnets.xyz/](https://status.teztnets.xyz/)
  - [https://explorus.functori.com/scoru](https://explorus.functori.com/scoru)

- Which wallet supports MondayNet?
  - Temple wallet

- How to **setup** MondayNet?
  
  There are several approach:
  - From Tezos source code:
    
    Download and compile the source code as following:
    ```shell
        cd tezos
        git checkout 8b50837b
        eval $(opam env)
        make build-dev-deps
        make
    ```
  - Flextesa:
    
    For Flextesa there is a "hack" version can be found at <https://github.com/lykimq/flextesa_mondaynet> that support the MondayNet. **Use at your own risk.**
  - Bakingsetup:
  
    This is a tool that contents scripts can be used to start and stop Tezos nodes and bakers that also support MondayNet. **Use at your own risk.**

    ```shell
    git clone https://github.com/drchrispinnock/bakingsetup
    ```

    Edit `/etc/hostname`, add at the beginning `mondaynet`.

    Setup mondaynet:
    ```shell
    cd mondaynet
    chmod 777 mondaynet-setup.sh
    ./mondaynet-setup.sh
    ```

- How to **config** `octez-node` on MondayNet?
  
  - From Tezos source code
    ```shell
      # config octez-node with the mondaynet network
      ./octez-node config init --network https://teztnets.xyz/mondaynet-2022-12-05
    ```

  - Flexetesa:
    - Create alias for `docker exec` as follows:
        ```shell
        # alias for octez-client
        alias oct_cli='docker exec alphabox octez-client'
        # alias for octez-node
        alias oct_node='docker exec alphabox octez-node'
        # alias for octez-sc-rollup-client-alpha
        alias sc_cli='docker exec alphabox octez-sc-rollup-client-alpha'
        # alias for octez-sc-rollup-node-alpha
        alias sc_node='docker exec alphabox octez-sc-rollup-node-alpha'
        ```
    - Config 
        ```shell
        oct_node config init --network https://teztnets.xyz/mondaynet-2022-12-05
        ```
   - Bakingsetup
    
      The bakingsetup will config and run the `octez-node`:
      ```shell
        cd mondaynet
        chmod 777 mondaynet-start.sh
        ./mondaynet-start.sh
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
   - The flextesa is similar as the one from Tezos source code.
   - The bakingsetup will start running the `octez-node` after using `./mondaynet-start.sh`.

- How to **add account** on MondayNet?

  Note that, we only show how to use the commands from Tezos source code, other approach will be similar.
  ```shell
  # generate new account
  ./octez-client gen keys alice
  # show 
  ./octez-client list known addresses
  ```

- How to add **faucet** to the account?
  
   The faucet for MondayNet can be found at <https://faucet.mondaynet-2022-12-05.teztnets.xyz>

   For instance, `alice` has an account: `tz1NCwYf8HF1V2nft3w657uwNQZPyMHA1xEM`. To add the faucet for `alice`, copy her address to the box in the section `Or fund any address` and request `6001tz` for her.

   To check if `alice` receives the fund:

   ```shell
   ./octez-client get balance for alice
   ```

   For the account from Temple wallet, connect to the wallet and then request the fund for it.
