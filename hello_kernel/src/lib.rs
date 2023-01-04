// SPDX-FileCopyrightText: 2022 TriliTech <contact@trili.tech>
// SPDX-FileCopyrightText: 2022 Nomadic Labs <contact@nomadic-labs.com>
// SPDX-FileCopyrightText: 2022 Marigold <contact@marigold.dev>
//
// SPDX-License-Identifier: MIT

use capnp::{
    message::{ReaderOptions, TypedReader},
    serialize_packed::read_message,
};
use host::rollup_core::{RawRollupCore, MAX_INPUT_MESSAGE_SIZE, MAX_INPUT_SLOT_DATA_CHUNK_SIZE};
use kernel::kernel_entry;

#[allow(unused)]
mod schema;

const INPUT_TRUNCATED: &str = "inbox message is too long";

pub fn main<H: RawRollupCore>(host: &mut H) {
    use host::runtime::Runtime as _;

    let Ok(maybe_input) = host.read_input(
        MAX_INPUT_MESSAGE_SIZE.max(MAX_INPUT_SLOT_DATA_CHUNK_SIZE)
    ) else {
        unsafe {
            <H as RawRollupCore>::write_debug(INPUT_TRUNCATED.as_ptr(), INPUT_TRUNCATED.len());
        }
        return;
    };
    let Some(input) = maybe_input else {
        // there is no input yet, skipping
        return;
    };
    let input_data = match &input {
        host::input::Input::Slot(data) => data.as_ref(),
        host::input::Input::Message(_) => {
            let message = "we do not accept L1 message";
            unsafe {
                <H as RawRollupCore>::write_debug(message.as_ptr(), message.len());
            }
            return;
        }
    };
    let reader;
    match read_message(input_data, ReaderOptions::new()) {
        Err(err) => panic!("read input: {err:?}"),
        Ok(r) => {
            reader = r;
        }
    }
    let reader = TypedReader::<_, schema::say_hello::Owned>::new(reader);
    let output;
    match reader.get().and_then(|r| r.get_name()) {
        Ok(name) => {
            output = format!("hello world, {name}");
        }
        Err(err) => panic!("parsing NAME: {err:?}"),
    }
    if let Err(err) = host.write_output(output.as_bytes()) {
        panic!("host: {err:?}")
    }
}

kernel_entry!(main);
