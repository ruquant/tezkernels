extern crate alloc;

use tezos_smart_rollup_entrypoint::kernel_entry;
use tezos_smart_rollup_host::runtime::Runtime;

mod counter;
use counter::*;

use tezos_smart_rollup_host::path::OwnedPath;

fn parse_input<Host: Runtime>(host: &mut Host) -> Result<Option<UserAction>, InputError> {
    // Read the input
    let input = host.read_input();
    match input {
        // If it's an error or no message then does nothing
        Err(e) => Err(InputError::Runtime(e)),
        Ok(None) => Err(InputError::NoMoreMessages),
        Ok(Some(message)) => {
            host.write_debug("Hello message\n");
            let data = message.as_ref();
            match data {
                [0x00, ..] => {
                    host.write_debug("Message from the kernel.\n");
                    Ok(None)
                }
                [0x01, ..] => {
                    host.write_debug("Message from the user.\n");
                    // Let's skip the first byte of the data to get what the user has sent.
                    let user_message: Vec<&u8> = data.iter().skip(1).collect();
                    // We are parsing the message from the user.
                    // In the case of a good encoding we can process it.
                    let user_message = UserAction::try_from(user_message);
                    match user_message {
                        Ok(user_message) => Ok(Some(user_message)),
                        Err(e) => Err(e),
                        }
                    }
                _ => Ok(None),
            }
        }
    }
}

fn execute<Host: Runtime>(host: &mut Host, counter: Counter) -> Counter {
    let input = parse_input(host);
    match input {
        Err(_) => counter,
        // If there is a message let's process it.
        Ok(input) => {
            let counter = match input {
                Some(user_message) => transition(counter, user_message),
                None => counter,
            };
            execute(host, counter)
        }
    }
}

fn entry(host: &mut impl Runtime) {
    let counter_path: OwnedPath = "/counter".as_bytes().to_vec().try_into().unwrap();
    let counter = Runtime::store_read(host, &counter_path, 0, 8)
        .map_err(|_| "Runtime error".to_string())
        .and_then(Counter::try_from)
        .unwrap_or_default();

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

#[cfg(test)]
mod test {
    use super::*;

    fn assert_eq_input<Host: Runtime>(host: &mut Host, action: UserAction) {
        let message = parse_input(host).unwrap();
        assert_eq!(message, Some(action));
    }


    #[test]
    fn test_parse_input() {
        let mut host = tezos_smart_rollup_mock::MockHost::default();

        host.add_external(UserAction::Increment);
        host.add_external(UserAction::Decrement);
        host.add_external(UserAction::Reset);

        assert_eq_input(&mut host, UserAction::Increment);
        assert_eq_input(&mut host, UserAction::Decrement);
        assert_eq_input(&mut host, UserAction::Reset);

        // TODO: test invalid input

        let message = parse_input(&mut host);
        assert!(matches!(message, Err(InputError::NoMoreMessages)));
    }

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

        host.add_external(UserAction::Increment);
        host.add_external(UserAction::Reset);
        host.add_external(UserAction::Increment);
        host.add_external(UserAction::Increment);
        host.add_external(UserAction::Decrement);
        host.run_level(entry);
        let counter = Runtime::store_read(&mut host, &counter_path, 0, 8)
            .map_err(|_| "Runtime error".to_string())
            .and_then(Counter::try_from)
            .unwrap_or_default();
        assert_eq!(counter, Counter { counter: 1 });
    }
}
