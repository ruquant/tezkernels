extern crate alloc;

use tezos_data_encoding::enc::BinWriter;
use tezos_smart_rollup_debug::debug_msg;
use tezos_smart_rollup_encoding::inbox::InboxMessage;
use tezos_smart_rollup_encoding::michelson::MichelsonUnit;
use tezos_smart_rollup_entrypoint::kernel_entry;
use tezos_smart_rollup_host::runtime::Runtime;

mod counter;
use counter::*;

use tezos_smart_rollup_host::path::OwnedPath;

fn execute(host: &mut impl Runtime, counter: Counter) -> Counter {
    // Read the input
    let input = host.read_input();

    match input {
        // If it's an error or no message then does nothing}
        Err(_) | Ok(None) => counter,
        Ok(Some(input)) => {
            // If there is a message let's process it.
            debug_msg!(host, "Hello message\n");
            let (_remaining, message) =
                InboxMessage::<MichelsonUnit>::parse(input.as_ref()).unwrap(); // TODO: handle error
            match message {
                InboxMessage::Internal(_) => {
                    debug_msg!(host, "Message from the runtime: {:?}\n", message);
                    counter
                }
                InboxMessage::External(user_message) => {
                    debug_msg!(host, "Message from the user: {:?}\n", user_message);
                    match UserAction::try_from(user_message) {
                        Ok(action) => counter::transition(counter, action),
                        Err(_) => counter,
                    }
                    // match UserAction::try_from(user_message) {
                    //     Ok(UserAction::Increment) => {
                    //         debug_msg!(host, "Incrementing counter.\n");
                    //     }
                    //     _ => panic!(""),
                    // }
                    // debug_msg!(host, "Message from the user.\n");
                    // We are parsing the message from the user.
                    // In the case of a good encoding we can process it.
                    // let user_message = UserAction::try_from(user_message);
                    // match user_message {
                    //     Ok(user_message) => {
                    //         // FIXME: should we perhaps inline this logic into
                    //         // this module?
                    //         let counter = transition(counter, user_message);
                    //         execute(host, counter)
                    //     }
                    //     Err(_) => execute(host, counter),
                    // }
                } // _ =>
                  // // FIXME: what is this case?
                  // // If these cases are well-defined, can the SDK return an enum instead
                  // // of the raw bytes?
                  // // E.g. `enum Message { Kernel(Vec<u8>), User(Vec<u8>) }`
                  // {
                  //     execute(host, counter)
                  // }
            }
        }
    }
}

fn entry(host: &mut impl Runtime) {
    let counter_path: OwnedPath = "/counter".as_bytes().to_vec().try_into().unwrap();
    let counter = Runtime::store_read(host, &counter_path, 0, 8)
        .map_err(|_| "Runtime error".to_string())
        .and_then(Counter::try_from)
        .unwrap_or_default();

    debug_msg!(host, "Hello kernel\n");
    let counter = execute(host, counter);

    let counter: [u8; 8] = counter.into();
    let _ = Runtime::store_write(host, &counter_path, &counter, 0);
    host.mark_for_reboot().unwrap();
}

kernel_entry!(entry);

// To run:
// 1. cargo build --release --target wasm32-unknown-unknown --features greeter
// 2. octez-smart-rollup-wasm-debugger target/wasm32-unknown-unknown/release/coutner_kernel.wasm --inputs ./counter_kernel/inputs.json
// 'load inputs'
// 'step result'
// 'show key /counter'

mod test {
    use super::*;

    #[test]
    fn test_counter() {
        let mut host = tezos_smart_rollup_mock::MockHost::default();

        let counter_path: OwnedPath = "/counter".as_bytes().to_vec().try_into().unwrap();
        host.run_level(entry);

        let counter = Runtime::store_read(&mut host, &counter_path, 0, 8)
            .map_err(|_| "Runtime error".to_string())
            .and_then(Counter::try_from)
            .unwrap_or_default();

        assert_eq!(counter, Counter { counter: 0 });

        let action = UserAction::Increment;
        host.add_external(action);
        host.run_level(entry);
        let counter = Runtime::store_read(&mut host, &counter_path, 0, 8)
            .map_err(|_| "Runtime error".to_string())
            .and_then(Counter::try_from)
            .unwrap_or_default();
        assert_eq!(counter, Counter { counter: 1 });
    }
}
