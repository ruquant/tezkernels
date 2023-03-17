extern crate alloc;

use debug::debug_msg;
use host::rollup_core::{RawRollupCore, MAX_INPUT_MESSAGE_SIZE};
use host::runtime::Runtime;
use kernel::kernel_entry;

/// The main entrypoint of the kernel.
///
/// This function is called by the runtime in a loop, and is
/// responsible for processing inputs (e.g. reading messages from the shared inbox
/// or revealing preimages of hashes) and writing to persistent storage and the shared
/// outbox.
///
/// Special care must be taken to ensure that the kernel does not run out of ticks
/// and that inputs are handled appropriately. We'll cover some of these topics
/// in coming examples, but we suggest having a look at the documentation as well:
/// https://tezos.gitlab.io/mumbai/smart_rollups.html#developing-wasm-kernels
fn entry<Host: RawRollupCore>(host: &mut Host) {
    // The `debug_msg!` macro prints messages that can be observed
    // when executing with the octez-smart-rollup-wasm-debugger binary.
    debug_msg!(Host, "Hello from kernel!!\n");

    // Read the next inbox message from the runtime.
    match host.read_input(MAX_INPUT_MESSAGE_SIZE) {
        Ok(Some(input)) => {
            let message = input.as_ref();
            // The first byte of the message is used to distinguish between
            // messages from the runtime and messages from the user.
            // TODO: include a table detailing the message types.
            match message.get(0) {
                Some(0x00) => {
                    debug_msg!(Host, "Message from the runtime: {:?}\n", message);
                }
                Some(0x01) => {
                    debug_msg!(
                        Host,
                        "Message from the user: {}.\n",
                        String::from_utf8_lossy(message)
                    );
                }
                _ => {
                    debug_msg!(Host, "Message from the unknown.\n");
                }
            }
        }
        Ok(None) => (),
        Err(err) => {
            debug_msg!(Host, "Error reading input: {:?}\n", err);
            ()
        }
    };
    // Marks a transient flag, instructing the runtime to immediately reboot the kernel
    // Without this flag, the kernel would not be rebooted until the start of the next
    // Tezos block level, and input messages for the current level might be skipped.
    host.mark_for_reboot().unwrap()
}

// Registers our `entry` function as the `kernel_run` function in the WASM output.
kernel_entry!(entry);
