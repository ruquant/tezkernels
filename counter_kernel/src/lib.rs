/*****************************************************************************/
/*                                                                           */
/* Open Source License                                                       */
/* Copyright (c) 2022 Nomadic Labs <contact@nomadic-labs.com>                */
/*                                                                           */
/* Permission is hereby granted, free of charge, to any person obtaining a   */
/* copy of this software and associated documentation files (the "Software"),*/
/* to deal in the Software without restriction, including without limitation */
/* the rights to use, copy, modify, merge, publish, distribute, sublicense,  */
/* and/or sell copies of the Software, and to permit persons to whom the     */
/* Software is furnished to do so, subject to the following conditions:      */
/*                                                                           */
/* The above copyright notice and this permission notice shall be included   */
/* in all copies or substantial portions of the Software.                    */
/*                                                                           */
/* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR*/
/* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,  */
/* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL   */
/* THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER*/
/* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING   */
/* FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER       */
/* DEALINGS IN THE SOFTWARE.                                                 */
/*                                                                           */
/*****************************************************************************/

use host::{
    rollup_core::{ RawRollupCore, MAX_INPUT_MESSAGE_SIZE, MAX_INPUT_SLOT_DATA_CHUNK_SIZE },
};
use host::input::{ Input };
use host::runtime::Runtime;
use debug::debug_msg;
/* Todo: use kernel_core atm */
use kernel_core::inbox::{ InboxMessage, InternalInboxMessage, Transfer };
use kernel_core::memory::Memory;

const MAX_READ_INPUT_SIZE: usize = if MAX_INPUT_MESSAGE_SIZE > MAX_INPUT_SLOT_DATA_CHUNK_SIZE {
    MAX_INPUT_MESSAGE_SIZE
} else {
    MAX_INPUT_SLOT_DATA_CHUNK_SIZE
};

// Counter
pub struct Counter {
    val: i8,
}

impl Counter {
    // Public read-only method: Returns the counter value
    pub fn get_num(&self) -> i8 {
        return self.val;
    }

    // increment the counter
    pub fn increment(&mut self) {
        self.val += 1;
    }

    pub fn decrement(&mut self) {
        self.val -= 1;
    }

    // reset to 0
    pub fn reset(&mut self) {
        self.val = 0;
    }
}

fn process_counter<'a, Host: RawRollupCore>(
    host: &mut Host,
    memory: &mut Memory,
    counter: &mut Counter,
    payload: &'a [u8]
) {
    /* parsing the Input message with the payload */
    let (_remaining, message) = InboxMessage::parse(payload).expect("Failed on parse payload");

    /* Input message is either external or internal message */
    match message {
        /* External message, it is hex */
        InboxMessage::External(message) => {
            debug_msg!(Host, "Received an external message {:?}", message);

            let incr_counter = counter.increment();
            debug_msg!(Host, "Counter {:#?}", incr_counter);
        }

        /* Internal message via contract call */
        InboxMessage::Internal(message) => {
            match message {
                InternalInboxMessage::Transfer(Transfer { payload, .. }) => {
                    debug_msg!(Host, "Received an internal message {:?}", payload);

                    let incr_counter = counter.increment();
                    debug_msg!(Host, "Counter {:#?}", incr_counter);
                }
                InternalInboxMessage::StartOfLevel => {
                    debug_msg!(Host, "Start of level");
                }
                InternalInboxMessage::EndOfLevel => {
                    debug_msg!(Host, "End of level");
                }
            }
        }
    }

    /* call memory snapshot */
    memory.snapshot(host);
}

pub fn counter_run<Host: RawRollupCore>(host: &mut Host, counter: &mut Counter) {
    let mut memory = Memory::load_memory(host);

    /* Reading the input from host */
    match host.read_input(MAX_READ_INPUT_SIZE) {
        Ok(Some(Input::Message(message))) => {
            debug_msg!(Host, "message data at level:{} - id:{}", message.level, message.id);
            /* processing counter function  */
            process_counter(host, &mut memory, counter, message.as_ref());
        }
        Ok(Some(Input::Slot(_message))) => todo!("handle slot message"),
        Ok(None) => debug_msg!(Host, "no input"),
        Err(_) => todo!("handle errors"),
    }

    /* memory is like a database, make a snapshot */
    memory.snapshot(host)
}

// Set up our own kernel_entry
#[macro_export]
macro_rules! kernel_entry {
    ($kernel_next:expr) => {
    #[no_mangle]
    pub extern "C" fn kernel_run() {
    use host::rollup_core::{ RawRollupCore };
    use host::wasm_host::WasmHost;
    let mut host = unsafe { host::wasm_host::WasmHost::new() };
    counter_run(host)
}
    };
}