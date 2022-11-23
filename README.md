# Marigold kernel

## Prerequired

- `wasm-strip` is a part of the [WebAssembly Binary Toolkit](https://github.com/WebAssembly/wabt) to strip down the size of wasm kernel.

### Install Rust

```shell
# [install rust]
wget https://sh.rustup.rs/rustup-init.sh
chmod +x rustup-init.sh
./rustup-init.sh --profile minimal --default-toolchain 1.60.0 -y
# [source cargo]
. $HOME/.cargo/env
```

- Add wasm target:

```shell
rustup target add wasm32-unknown-unknown
```

## Gitbook

Gitbook of [pistachio](https://app.gitbook.com/o/Gayxsw4YmVrLK4YRDlmi/s/bQv7Nn2dfUKSfyTtFM1M/pistachio/kernel-in-scoru).

# Footnotes

The logo of this project is the [Pistachio icons created by Freepik - Flaticon](https://www.flaticon.com/free-icons/pistachio).
