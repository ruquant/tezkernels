/*****************************************************************************/
/*                                                                           */
/* Open Source License                                                       */
/* Copyright (c) 2023 Marigold <contact@marigold.dev>                        */
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

use capnp::{
    message::{ReaderOptions, TypedReader},
    serialize_packed::read_message,
};
use host::rollup_core::{RawRollupCore, MAX_INPUT_MESSAGE_SIZE, MAX_INPUT_SLOT_DATA_CHUNK_SIZE};
#[cfg(not(test))]
use kernel::kernel_entry;
use kernel_core::inbox::InboxMessage;

#[allow(unused)]
mod schema;

const INPUT_TRUNCATED: &str = "inbox message is too long";

pub fn main<H: RawRollupCore>(host: &mut H) {
    use host::runtime::Runtime as _;

    // In this loop, we will read through all messages in the inbox
    loop {
        let Ok(maybe_input) = host.read_input(
            MAX_INPUT_MESSAGE_SIZE.max(MAX_INPUT_SLOT_DATA_CHUNK_SIZE)
        ) else {
            unsafe {
                <H as RawRollupCore>::write_debug(INPUT_TRUNCATED.as_ptr(), INPUT_TRUNCATED.len());
            }
            // Skip this message because it is malformed
            continue;
        };
        let Some(input) = maybe_input else {
            // There is no more input in this inbox.
            // We are done here.
            return;
        };
        let input_data = match &input {
            host::input::Input::Slot(data) => data.as_ref(),
            host::input::Input::Message(data) => {
                let (_, data) = InboxMessage::parse(data.as_ref()).unwrap();
                // We only care about external messages.
                match data {
                    InboxMessage::External(ext) => ext.0,
                    _ => continue,
                }
            }
        };
        let reader;
        // If we cannot decode the compacted CapNProto message,
        // this message is probably not directed at this kernel.
        match read_message(input_data, ReaderOptions::new()) {
            Err(_) => continue,
            Ok(r) => {
                reader = r;
            }
        }
        let reader = TypedReader::<_, schema::say_hello::Owned>::new(reader);
        let output;
        // If we cannot parse the message into CapNProto,
        // this message is probably not for us.
        match reader.get().and_then(|r| r.get_name()) {
            Ok(name) => {
                output = format!("hello world, {name}");
            }
            Err(_) => continue,
        }
        if let Err(err) = host.write_output(output.as_bytes()) {
            panic!("host: {err:?}")
        }
    }
}

#[cfg(not(test))]
kernel_entry!(main);

#[cfg(test)]
mod tests {
    use super::*;
    use capnp::{message::TypedBuilder, serialize_packed::write_message};
    use host::rollup_core::Input;
    use mock_runtime::{host::MockHost, state::HostState};
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn smoke_test() {
        let mut state = HostState::default();
        state.set_ready_for_input(0);
        state.add_next_inputs(
            0,
            [
                (Input::MessageData, {
                    let mut builder = TypedBuilder::<schema::say_hello::Owned>::new_default();
                    builder.init_root().set_name("Tezos");
                    let mut buf = vec![1];
                    write_message(&mut buf, builder.borrow_inner()).unwrap();
                    buf
                }),
                (Input::MessageData, {
                    let mut builder = TypedBuilder::<schema::say_hello::Owned>::new_default();
                    builder.init_root().set_name("Marigold");
                    let mut buf = vec![1];
                    write_message(&mut buf, builder.borrow_inner()).unwrap();
                    buf
                }),
            ]
            .iter(),
        );
        let mut host = MockHost::from(state);
        main(&mut host);
        let state = host.into_inner();
        let value: Vec<_> = state.store.get_value("/output/0/0");
        assert_eq!(value, b"hello world, Tezos");
        let value: Vec<_> = state.store.get_value("/output/0/1");
        assert_eq!(value, b"hello world, Marigold");
    }
}
