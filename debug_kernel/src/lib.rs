extern crate alloc;

use debug::debug_msg;
use host::rollup_core::{RawRollupCore, MAX_INPUT_MESSAGE_SIZE};
use host::runtime::Runtime;
use kernel::kernel_entry;

// To run:
// 1. cargo build --release --target wasm32-unknown-unknown --features greeter
// 2. octez-smart-rollup-wasm-debugger target/wasm32-unknown-unknown/release/debug_kernel.wasm --inputs ./debug_kernel/inputs.json
// 'load inputs'
// 'step result'
fn entry< Host: RawRollupCore>(host: &mut Host) {
    debug_msg!(Host,"Hello from kernel_run!\n");
    loop {
        match host.read_input(MAX_INPUT_MESSAGE_SIZE) {
            Ok(Some(input)) => {
                let message = input.as_ref();
                match message.get(0) {
                    Some(0x00) => {
                        debug_msg!(Host, "Message from the runtime: {:?}\n", message);
                    }
                    Some(0x01) => {
                        debug_msg!(Host, "Message from the user: {}.\n", String::from_utf8_lossy(message));
                    }
                    _ => {
                        debug_msg!(Host, "Message from the unknown.\n");
                    }
                }
            }
            Ok(None) => break,
            Err(err) => {
                debug_msg!(Host, "Error reading input: {:?}\n", err);
                break;
            }
        }
    }
}

kernel_entry!(entry);
