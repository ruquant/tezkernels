extern crate alloc;

use debug::debug_msg;
use host::rollup_core::{RawRollupCore, MAX_INPUT_MESSAGE_SIZE};
use host::runtime::Runtime;
use kernel::kernel_entry;

mod counter;
use counter::*;

use host::path::OwnedPath;

fn execute<Host: RawRollupCore>(host: &mut Host, counter: Counter) -> Counter {
    // Read the input
    let input = host.read_input(MAX_INPUT_MESSAGE_SIZE);

    match input {
        // If it's an error or no message then does nothing}
        Err(_) | Ok(None) => counter,
        Ok(Some(message)) => {
            // If there is a message let's process it.
            debug_msg!(Host,"Hello message\n");
            let data = message.as_ref();
            match data {
                [0x00, ..] => {
                    debug_msg!(Host,"Message from the kernel.\n");
                    execute(host, counter)
                }
                [0x01, ..] => {
                    debug_msg!(Host,"Message from the user.\n");
                    // Let's skip the first byte of the data to get what the user has sent.
                    let user_message: Vec<&u8> = data.iter().skip(1).collect();
                    // We are parsing the message from the user.
                    // In the case of a good encoding we can process it.
                    let user_message = UserAction::try_from(user_message);
                    match user_message {
                        Ok(user_message) => {
                            // FIXME: should we perhaps inline this logic into
                            // this module?
                            let counter = transition(counter, user_message);
                            execute(host, counter)
                        }
                        Err(_) => execute(host, counter),
                    }
                }
                _ =>
                // FIXME: what is this case?
                // If these cases are well-defined, can the SDK return an enum instead
                // of the raw bytes?
                // E.g. `enum Message { Kernel(Vec<u8>), User(Vec<u8>) }`
                execute(host, counter),
            }
        }
    }
}

fn entry< Host: RawRollupCore>(host: &mut Host) {
    let counter_path: OwnedPath = "/counter".as_bytes().to_vec().try_into().unwrap();
    let counter = Runtime::store_read(host, &counter_path, 0, 8)
        .map_err(|_| "Runtime error".to_string())
        .and_then(Counter::try_from)
        .unwrap_or_default();

    debug_msg!(Host,"Hello kernel\n");
    let counter = execute(host, counter);

    let counter: [u8; 8] = counter.into();
    let _ = Runtime::store_write(host, &counter_path, &counter, 0);
}

kernel_entry!(entry);

// To run:
// 1. cargo build --release --target wasm32-unknown-unknown --features greeter
// 2. octez-smart-rollup-wasm-debugger target/wasm32-unknown-unknown/release/coutner_kernel.wasm --inputs ./counter_kernel/inputs.json
// 'load inputs'
// 'step result'
// 'show key /counter'
